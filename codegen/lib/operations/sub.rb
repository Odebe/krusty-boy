require_relative './base'

module Operations
  class SUB < Base
    def self.template
      ERB.new <<~EOF
        cpu.reg.a = cpu.alu_sub(cpu.reg.a, <%= @op1_builder.call %>)
      EOF
    end
  end
end
