require_relative './base'

module Operations
  class ADC < Base
    def self.template
      ERB.new <<~EOF
        let a = <%= @op1_builder.call %>;
        let b = <%= @op2_builder.call %>;

        let value = <%= add_func_call %>;

          <%= call %>;
      EOF
    end

    def add_func_call
      "cpu.alu_adc(a, b)"
    end

    def call
      "cpu.registers.set_a(value)"
    end
  end
end
