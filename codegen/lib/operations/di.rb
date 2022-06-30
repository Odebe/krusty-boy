require_relative './base'

module Operations
  class DI < Base
    def self.template
      ERB.new <<~EOF
        cpu.ei = false;
      EOF
    end
  end
end
