module Strategy
  module Read
    class Pointer
      def self.call(operand)
        "cpu.#{operand.clean_name.downcase}"
      end
    end
  end
end
