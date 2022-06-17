require_relative './base'

module Operations
  class RLCA < Base
    def self.template
      ERB.new <<~EOF
        cpu.reg.a = cpu.alu_rlc(<%= op1_builder.call %>);
        cpu.reg.flag_set(N, false);
      EOF
    end

    def op1_builder
      OperandBuilder.new('', 'A')
    end
  end
end
