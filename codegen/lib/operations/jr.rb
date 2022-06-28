require_relative './base'

module Operations
  class JR < Base
    def self.template
      ERB.new <<~EOF
        let delta = cpu.read_n() as i8; 

<% if operand2.present? %> if <%= operand1.render_as(::Strategy::Read::Flag) %> { <% end %>
        cpu.jr(delta);
<% if operand2.present? %> } <% end %>
      EOF
    end
  end
end
