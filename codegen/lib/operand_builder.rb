require_relative 'operand'

Dir[File.dirname(__FILE__) + '/strategy/read/*.rb'].each do |file|
  require_relative file
end

Dir[File.dirname(__FILE__) + '/strategy/write/*.rb'].each do |file|
  require_relative file
end

require_relative 'render/operand'

class OperandBuilder
  attr_reader :operand

  def initialize(key, mnem, only: false)
    @only = only
    @operand = Operand.new(key, mnem)
  end

  def call
    @operand.render_as(read_strategy)
  end

  def read_strategy
    base =
      if @operand.register?
        ::Strategy::Read::Register
      elsif @operand.pointer?
        ::Strategy::Read::Pointer
      elsif @operand.number?
        ::Strategy::Read::Number
      else
        ::Strategy::Read::Simple
      end

    base = ::Strategy::Read::HalfWord.new(base) if @operand.half_word?

    if (@operand.indirect? && @only) || (@operand.indirect? && @operand.key == 'operand2')
      ::Strategy::Read::Indirect.new(base)
    else
      base
    end
  end
end
