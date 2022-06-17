require_relative './base'

module Operations
  class ADD < Base
    def u8_template
      ERB.new <<~EOF
        cpu.reg.a = #{call};
      EOF
    end

    def u16_template
      ERB.new <<~EOF
<% if operand1.pointer? %>
        cpu.#{operand1.clean.downcase} = #{call};
<% else %>
        cpu.reg.set_#{operand1.clean.downcase}(#{call});
<% end %>
      EOF
    end

    def call
      "cpu.alu_add_u8(#{@op1_builder.call}, #{@op2_builder.call})"
    end
  end
end
