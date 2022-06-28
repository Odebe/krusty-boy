require_relative './base'

module Operations
  module BIT
    class X8 < Base
      def template
        ERB.new <<~EOF
          cpu.alu_bit(<%= @op2_builder.call %>, <%= @op1_builder.call %>);
        EOF
      end
    end
  end
end

