require_relative './base'

module Operations
  module AND
    class X8 < Base
      def template
        ERB.new <<~EOF
          cpu.a = cpu.alu_and(cpu.reg.a, <%= @op1_builder.call %>);
        EOF
      end
    end
  end
end
