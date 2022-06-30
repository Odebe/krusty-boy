require_relative './base'

module Operations
  module INC
    class X8 < Base
      def template
        ERB.new <<~EOF
<% if operand1.indirect? %>
          let addr = #{operand1.render_as(::Strategy::Read::Register)};
          let value = cpu.alu_inc(cpu.read_u8(addr));
<% else %>
          let value = cpu.alu_inc(#{@op1_builder.call});
<% end %>
          <%= write_u8 %>;
        EOF
      end
    end

    class X16 < Base
      def template
        ERB.new <<~EOF
          let value = cpu.alu_inc16(#{@op1_builder.call});
          <%= write_u16 %>;
        EOF
      end
    end
  end
end
