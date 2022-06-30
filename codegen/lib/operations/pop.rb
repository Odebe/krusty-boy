require_relative './base'

module Operations
  class POP < Base
    def self.template
      ERB.new <<~EOF
        <% value = "cpu.stack_pop()" %>
        <%= write_u16(value) %>;
      EOF
    end
  end
end
