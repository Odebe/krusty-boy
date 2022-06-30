class Operand
  TRANSLATIONS = {
    'd8' => 'n',
    'r8' => 'd',
    'a8' => 'n',
    'd16' => 'nn',
    'a16' => 'nn'
  }

  attr_reader :key

  def initialize(key, mnem)
    @key = key
    @mnem = mnem || ''
  end

  def present?
    @mnem != ''
  end

  def render_as(strategy)
    strategy.call(self)
  end

  def _translate(key)
    TRANSLATIONS[key] || key
  end

  def clean
    @clean ||= @mnem.gsub(/[\(\)\+\-]/, '')
  end

  def addr?
    indirect? || (!number? && !register? && !pointer? && !flag?)
  end

  def half_word?
    @mnem.include?('a8')
  end

  def indirect?
    /^\(\w{1,3}\)$/.match?(@mnem)
  end

  def number?
    /^\d$/.match?(@mnem)
  end

  def pointer?
    %w[SP PC].include?(clean)
  end

  def register?
    %w[A F B C BC D E DE H L HL].include?(clean)
  end

  def flag?
    %w[Z NZ C NC].include?(clean)
  end

  def composed?
    composition.any?
  end

  def composition
    @composition ||= /^(\w{1,2})(\+|\-)(\w{0,2})$/.match(clean_name)[1..-1]
  end

  def clean_flag
    clean_name.gsub(/N/, '')
  end

  def negative_flag?
    flag? && clean[0] == 'N'
  end

  def u16?
    source_name.chars.count == 2
  end

  def u8?
    source_name.chars.count == 1
  end

  def source_name
    @source_name ||= _translate(clean)
  end

  def clean_name
    @clean ||= @mnem.gsub(/[\(\)]/, '')
  end
end
