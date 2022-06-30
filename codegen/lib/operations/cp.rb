require_relative './base'

module Operations
  module CP
    class X8 < Base
      def template
        ERB.new <<~EOF
          cpu.reg.a = cpu.alu_cp(cpu.reg.a, <%= @op1_builder.call %>);
        EOF
      end
    end
  end
end

