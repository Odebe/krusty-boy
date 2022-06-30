require_relative './base'

module Operations
  module ADD
    class X8 < Base
      def template
        ERB.new <<~EOF
          cpu.reg.a = cpu.alu_add_u8(#{@op1_builder.call}, #{@op2_builder.call});
        EOF
      end
    end

    class X16 < Base
      def template
        ERB.new <<~EOF
          let value = cpu.alu_add_u16(#{@op1_builder.call}, #{@op2_builder.call});
          <%= write_u16 %>;
        EOF
      end
    end
  end
end
