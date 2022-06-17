module Strategy
  module Read
    class Number
      def self.call(operand)
        operand.clean_name
      end
    end
  end
end
