require_relative '../operand'

module Operations
  class Base
    attr_reader :operand1, :operand2

    def initialize(op1_builder, op2_builder)
      @op1_builder = op1_builder
      @op2_builder = op2_builder
    end

    def build
      select_template.result(binding)
    end

    private

    def select_template
      if operand1.u8? && respond_to?(:u8_template)
        u8_template
      elsif operand1.u16? && respond_to?(:u16_template)
        u16_template
      else
        self.class.template
      end
    end

    def operand1
      @op1_builder.operand
    end

    def operand2
      @op2_builder.operand
    end
  end
end
