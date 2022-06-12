require_relative './base'

module Operations
  class SBC < Base
    def self.template
      ERB.new <<~EOF
        let a = <%= @op1_builder.call %>;

        let value = <%= add_func_call %>;

          <%= call %>;
      EOF
    end

    def add_func_call
      "cpu.alu_sub(a)"
    end

    def call
      "cpu.registers.set_a(value)"
    end
  end
end
