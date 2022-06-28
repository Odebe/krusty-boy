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
      if respond_to?(:template)
        template
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

    def write_u8(value = 'value')
      if operand1.addr?
        "cpu.mmu.write_u8(addr, #{value})"
      elsif operand1.register?
        "cpu.reg.#{operand1.clean.downcase} = #{value}"
      else
        'compile_error!()'
      end
    end

    def write_u16(value = 'value')
      if operand1.addr?
        "cpu.mmu.write_u16(addr, #{value})"
      elsif operand1.pointer?
        "cpu.#{operand1.clean.downcase} = #{value}"
      elsif operand1.register?
        "cpu.reg.set_#{operand1.clean.downcase}(#{value})"
      else
        'compile_error!()'
      end
    end
  end
end
