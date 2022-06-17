require_relative './base'

module Operations
  class RLC < Base
    def self.template
      ERB.new <<~EOF
      <% if operand1.indirect? %>
        let addr = <%= operand1.render_as(::Strategy::Read::Register) %>;
        let op1 = cpu.mmu.read_u8(addr);
      <% else %>
        let op1 = <%= @op1_builder.call %>;
      <% end %>

        let value = <%= add_func_call %>;

          <%= add_write_func_call %>;
      EOF
    end

    def add_func_call
      "cpu.alu_rlc(op1)"
    end

    def add_write_func_call
      if operand1.indirect?
        "cpu.mmu.write_u8(addr, value)"
      elsif operand1.register? && operand1.u16?
        "cpu.reg.set_#{operand1.clean.downcase}(value)"
      elsif operand1.register? && operand1.u8?
        "cpu.reg.#{operand1.clean.downcase} = value"
      else
        'compile_error!()'
      end
    end
  end
end
