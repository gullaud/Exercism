let workDaysHours = 8.0
let monthWorkDays = 22.0

func dailyRateFrom(hourlyRate: Int) -> Double {
  return Double(hourlyRate) * workDaysHours
}

// What the discount means and what it applies to is poorly explained.
func monthlyRateFrom(hourlyRate: Int, withDiscount discount: Double) -> Double {
  return (dailyRateFrom(hourlyRate:hourlyRate) * monthWorkDays * (1.0 - discount * 0.01)).rounded()
}

func workdaysIn(budget: Double, hourlyRate: Int, withDiscount discount: Double) -> Double {
  return (budget / monthlyRateFrom(hourlyRate:hourlyRate, withDiscount:discount) * monthWorkDays).rounded(.down)
}
