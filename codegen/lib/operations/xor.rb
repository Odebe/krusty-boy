require_relative './base'

module Operations
  class XOR < Base
    def self.template
      ERB.new <<~EOF
        let a = cpu.registers.a;
        let b = <%= @op1_builder.call %>;

        let value = <%= add_func_call %>;

          <%= call %>;
      EOF
    end

    def add_func_call
      "cpu.alu_xor(a, b)"
    end

    def call
      "cpu.registers.set_a(value)"
    end
  end
end
