require_relative './base'

module Operations
  class SBC < Base
    def self.template
      ERB.new <<~EOF
        cpu.reg.a = cpu.alu_sbc(<%= @op1_builder.call %>);
      EOF
    end
  end
end
