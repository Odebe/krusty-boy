class Operand
  TRANSLATIONS = {
    'd8' => 'n',
    'r8' => 'u8',
    'd16' => 'nn',
    'a16' => 'nn'
  }

  def initialize(key, mnem)
    @key = key
    @mnem = mnem || ''
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
    indirect? || (!number? && !register? && !pointer?)
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
    %w[A B C BC D E DE H L HL].include?(clean)
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
