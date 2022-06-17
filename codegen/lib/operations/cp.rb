require_relative './base'

module Operations
  class CP < Base
    def self.template
      ERB.new <<~EOF
        cpu.reg.a = cpu.alu_cp(cpu.reg.a, <%= @op1_builder.call %>)
      EOF
    end
  end
end
