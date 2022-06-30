require_relative './base'

module Operations
  class CPL < Base
    def self.template
      ERB.new <<~EOF
        cpu.alu_cpl();
      EOF
    end
  end
end
