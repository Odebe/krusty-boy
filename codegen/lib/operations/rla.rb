require_relative './base'

module Operations
  class RLA < Base
    def self.template
      ERB.new <<~EOF
        cpu.reg.a = <%= add_func_call %>;
        cpu.reg.flag_set(N, false);
      EOF
    end

    def add_func_call
      "cpu.alu_rl(#{op1_builder.call})"
    end

    def op1_builder
      OperandBuilder.new('', 'A')
    end
  end
end
