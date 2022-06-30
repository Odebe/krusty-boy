require_relative './base'

module Operations
  class CCF < Base
    def self.template
      ERB.new <<~EOF
        cpu.alu_ccf();
      EOF
    end
  end
end
