require_relative './base'

module Operations
  class ADD < Base
    def self.template
      ERB.new <<~EOF
        let a = <%= @op1_builder.call %>;
        let b = <%= @op2_builder.call %>;

        let value = <%= add_func_call %>;

          <%= call %>;
      EOF
    end

    def add_func_call
      if operand2.u8?
        "cpu.alu_add_u8(a, b)"
      elsif operand2.u16?
        "cpu.alu_add_u16(a, b)"
      else
        'compile_error!()'
      end
    end

    def call
      if operand1.pointer?
        "cpu.set_#{operand1.clean.downcase}(value)"
      elsif operand1.register?
        "cpu.registers.set_#{operand1.clean.downcase}(value)"
      else
        'compile_error!()'
      end
    end
  end
end
