require_relative './base'

module Operations
  class EI < Base
    def self.template
      ERB.new <<~EOF
        cpu.ei = true;
      EOF
    end
  end
end
