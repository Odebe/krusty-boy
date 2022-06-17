require_relative './base'

module Operations
  class SET < Base
    def self.template
      ERB.new <<~EOF
      <% if operand2.indirect? %>
        let addr = <%= operand2.render_as(::Strategy::Read::Register) %>;
        let op1 = cpu.mmu.read_u8(addr);
      <% else %>
        let op1 = <%= @op2_builder.call %>;
      <% end %>

        let value = <%= add_func_call %>;

        <%= add_write_func_call %>;
      EOF
    end

    def add_func_call
      "cpu.alu_set(op, #{@op1_builder.call})"
    end

    def add_write_func_call
      if operand2.indirect?
        "cpu.mmu.write_u8(addr, value)"
      elsif operand2.register? && operand2.u16?
        "cpu.reg.set_#{operand2.clean.downcase}(value)"
      elsif operand2.register? && operand2.u8?
        "cpu.reg.#{operand2.clean.downcase} = value"
      else
        'compile_error!()'
      end
    end
  end
end
