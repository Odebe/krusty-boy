require_relative './base'

module Operations
  class LD < Base

    def u8_template
      if operand1.addr?
        ERB.new <<~EOF
          let addr = <%= @op1_builder.call %>;
          let value = <%= @op2_builder.call %>;
    
          <%= write %>;
        EOF
      else
        ERB.new <<~EOF
          let value = <%= @op2_builder.call %>;

          <%= write %>;
        EOF
      end
    end

    def u16_template
      ERB.new <<~EOF
          <% if operand1.addr? %> let addr = <%= @op1_builder.call %>; <% end %>
          let value = <%= @op2_builder.call %>;

          <%= write %>;
      EOF
    end

    def write
      if operand1.addr? && operand2.u8?
        "cpu.mmu.write_u8(addr, value)"
      elsif operand1.pointer?
        "cpu.#{operand1.clean.downcase} = value"
      elsif operand1.register?
        if operand1.u8?
          "cpu.reg.#{operand1.clean.downcase} = value"
        else
          "cpu.reg.set_#{operand1.clean.downcase}(value)"
        end
      else
        "cpu.write(addr, value)"
      end
    end
  end
end
