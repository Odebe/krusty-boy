require_relative './base'

module Operations
  module DEC
    class X8 < Base
      def template
        ERB.new <<~EOF
<% if operand1.indirect? %>
          <%= write_u8(value) %>;
<% else %>
          <%= write_u8(value) %>;
<% end %>
        EOF
      end

      def value
        if operand1.indirect?
          "cpu.alu_inc(cpu.read_u8(#{addr}))"
        else
          "cpu.alu_dec(#{@op1_builder.call})"
        end
      end

      def addr
        operand1.render_as(::Strategy::Read::Register)
      end
    end

    class X16 < Base
      def template
        ERB.new <<~EOF
          <%= write_u16(value) %>;
        EOF
      end

      def value
        "cpu.alu_dec16(#{@op1_builder.call})"
      end
    end
  end
end

