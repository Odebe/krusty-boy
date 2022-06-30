require_relative './base'

module Operations
  module LD
    class X8 < Base
      def template
        if operand1.addr?
          ERB.new <<~EOF
            let addr = <%= @op1_builder.call %>;
            let value = <%= @op2_builder.call %>;
            <%= write_u8 %>;
          EOF
        else
          ERB.new <<~EOF
            let value = <%= @op2_builder.call %>;
            <%= write_u8 %>;
          EOF
        end
      end
    end

    class X16 < Base
      def template
        ERB.new <<~EOF
          <% if operand1.addr? %> let addr = <%= @op1_builder.call %>; <% end %>
          let value = <%= @op2_builder.call %>;
          <%= write_u16 %>;
        EOF
      end
    end
  end
end
