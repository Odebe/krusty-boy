require_relative './base'

module Operations
  class STOP < Base
    def self.template
      ERB.new <<~EOF
        cpu.stop();
      EOF
    end
  end
end
