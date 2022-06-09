require_relative 'operand'

Dir[File.dirname(__FILE__) + '/strategy/read/*.rb'].each do |file|
  require_relative file
end

require_relative 'render/operand'

class OperandBuilder
  attr_reader :operand

  def initialize(key, mnem)
    @operand = Operand.new(key, mnem)
  end

  def call
    @operand.render_as(render_strategy)
  end

  def render_strategy
    base =
      if @operand.register?
        ::Strategy::Read::Register
      elsif @operand.pointer?
        ::Strategy::Read::Pointer
      else
        ::Strategy::Read::Simple
      end

    if @operand.indirect?
      ::Strategy::Read::Indirect.new(base)
    else
      base
    end
  end
end
