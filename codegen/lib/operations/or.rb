require_relative './base'

module Operations
  class OR < Base
    def self.template
      ERB.new <<~EOF
        <%= call %>;
      EOF
    end

    def add_func_call
      "cpu.alu_or(cpu.reg.a, #{@op1_builder.call})"
    end

    def call
      "cpu.reg.a = #{add_func_call}"
    end
  end
end
