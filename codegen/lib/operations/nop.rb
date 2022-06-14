require_relative './base'

module Operations
  class NOP < Base
    def self.template
      ERB.new <<~EOF

      EOF
    end
  end
end
