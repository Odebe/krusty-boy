require_relative './base'

module Operations
  class DAA < Base
    def self.template
      ERB.new <<~EOF
        cpu.alu_daa();
      EOF
    end
  end
end
