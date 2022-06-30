require_relative './base'

module Operations
  class RES < Base
    def self.template
      ERB.new <<~EOF
      <% if operand2.indirect? %>
        let addr = <%= operand2.render_as(::Strategy::Read::Register) %>;
        let op1 = cpu.mmu.read_u8(addr);

        let value = <%= add_func_call %>;
      <% else %>
        let value = cpu.alu_res(<%= @op2_builder.call %>, <%= @op1_builder.call %>);
      <% end %>

        <%= add_write_func_call %>;
      EOF
    end

    def add_func_call
      "cpu.alu_res(op1, #{@op1_builder.call})"
    end

    def add_write_func_call
      if operand2.indirect?
        "cpu.mmu.write_u8(addr, value)"
      elsif operand2.register?
        "cpu.reg.#{operand2.clean.downcase} = value"
      else
        'compile_error!()'
      end
    end
  end
end
