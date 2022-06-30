require_relative './base'

module Operations
  class PUSH < Base
    def self.template
      ERB.new <<~EOF
        cpu.stack_push(<%= @op1_builder.call %>);
      EOF
    end
  end
end
