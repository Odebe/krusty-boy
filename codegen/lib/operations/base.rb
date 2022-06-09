module Operations
  class Base
    attr_reader :operand1, :operand2

    def initialize(op1_builder, op2_builder)
      @op1_builder = op1_builder
      @op2_builder = op2_builder
    end

    def build
      self.class.template.result(binding)
    end

    private

    def operand1
      @op1_builder.operand
    end

    def operand2
      @op2_builder.operand
    end
  end
end
