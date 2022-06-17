require_relative './base'

module Operations
  class BIT < Base
    def self.template
      ERB.new <<~EOF
        cpu.alu_bit(<%= @op2_builder.call %>, <%= @op1_builder.call %>);
      EOF
    end
  end
end
