require_relative './base'

module Operations
  class AND < Base
    def self.template
      ERB.new <<~EOF
        cpu.a = cpu.alu_and(cpu.reg.a, <%= @op1_builder.call %>)";
      EOF
    end
  end
end
