require_relative './base'

module Operations
  class HALT < Base
    def self.template
      ERB.new <<~EOF
        cpu.halt();
      EOF
    end
  end
end
