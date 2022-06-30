module Strategy
  module Read
    class HalfWord
      def initialize(base)
        @base = base
      end

      def call(operand)
        "0xFF00 | u16::from(#{@base.call(operand)})"
      end
    end
  end
end
