module Strategy
  module Read
    class Register
      def self.call(operand)
        clean_name = operand.clean_name.downcase

        tmp = "cpu.registers.#{clean_name}"
        tmp << '()' if clean_name.chars.count > 1
        tmp
      end
    end
  end
end
