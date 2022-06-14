require_relative './base'

module Operations
  class RRA < Base
    def self.template
      ERB.new <<~EOF
        let a = <%= op1_builder.call %>;

        let value = cpu.alu_rlc(a);
        
        <%= call %>;

        cpu.n_flag = false;
      EOF
    end

    def add_func_call
      "cpu.alu_rr(a)"
    end

    def call
      "cpu.registers.set_a(value)"
    end

    def op1_builder
      OperandBuilder.new('', 'A')
    end
  end
end
