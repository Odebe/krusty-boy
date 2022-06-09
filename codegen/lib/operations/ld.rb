require_relative './base'

module Operations
  class LD < Base
    def self.template
      ERB.new <<~EOF
        <% if operand1.addr? %> let addr = <%= @op1_builder.call %>; <% end %>
        let value = <%= @op2_builder.call %>;

        <%= call %>;
      EOF
    end

    def call
      if operand1.addr? && operand2.u16?
        "cpu.mmu.write_u16(addr, value)"
      elsif operand1.addr? && operand2.u8?
        "cpu.mmu.write_u8(addr, value)"
      elsif operand1.pointer?
        "cpu.set_#{operand1.clean.downcase}(value)"
      elsif operand1.register?
        "cpu.registers.set_#{operand1.clean.downcase}(value)"
      else
        "cpu.write(addr, value)"
      end
    end
  end
end
