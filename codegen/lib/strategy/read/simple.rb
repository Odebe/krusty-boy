module Strategy
  module Read
    class Simple
      def self.call(operand)
        "cpu.read_#{operand.source_name.downcase}()"
      end
    end
  end
end
