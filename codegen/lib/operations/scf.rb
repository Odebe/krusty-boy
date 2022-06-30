require_relative './base'

module Operations
  class SCF < Base
    def self.template
      ERB.new <<~EOF
        cpu.alu_scf();
      EOF
    end
  end
end
