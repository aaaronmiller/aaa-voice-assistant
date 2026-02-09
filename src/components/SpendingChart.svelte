<script>
  import { onMount, afterUpdate } from 'svelte'
  import { Chart as ChartJS, ArcElement, Tooltip, Legend, CategoryScale, LinearScale, BarElement, Title } from 'chart.js'

  export let expenses = []

  // Register ChartJS components
  ChartJS.register(ArcElement, Tooltip, Legend, CategoryScale, LinearScale, BarElement, Title)

  let pieChart = null
  let barChart = null
  let pieCtx = null
  let barCtx = null

  // Calculate spending by category
  function calculateSpendingByCategory(expenses) {
    const spending = {}
    expenses.forEach(expense => {
      if (!spending[expense.category]) {
        spending[expense.category] = 0
      }
      spending[expense.category] += expense.amount
    })
    return spending
  }

  // Chart data for pie chart
  function getPieData() {
    const spendingByCategory = calculateSpendingByCategory(props.expenses)
    const categories = Object.keys(spendingByCategory)
    const amounts = Object.values(spendingByCategory)

    return {
      labels: categories,
      datasets: [
        {
          data: amounts,
          backgroundColor: [
            '#ef4444',
            '#f59e0b',
            '#10b981',
            '#3b82f6',
            '#6366f1'
          ],
          borderWidth: 1
        }
      ]
    }
  }

  // Chart data for bar chart
  function getBarData() {
    const spendingByCategory = calculateSpendingByCategory(props.expenses)
    const categories = Object.keys(spendingByCategory)
    const amounts = Object.values(spendingByCategory)

    return {
      labels: categories,
      datasets: [
        {
          label: 'Spending',
          data: amounts,
          backgroundColor: 'rgba(102, 126, 234, 0.8)',
          borderColor: 'rgba(102, 126, 234, 1)',
          borderWidth: 1
        }
      ]
    }
  }

  // Chart options
  const pieOptions = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        position: 'bottom'
      }
    }
  }

  const barOptions = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        display: false
      }
    },
    scales: {
      y: {
        beginAtZero: true,
        ticks: {
          callback: function(value) {
            return '$' + value
          }
        }
      }
    }
  }

  // Initialize charts
  onMount(() => {
    if (pieCtx && !pieChart) {
      pieChart = new ChartJS(pieCtx, {
        type: 'pie',
        data: getPieData(),
        options: pieOptions
      })
    }

    if (barCtx && !barChart) {
      barChart = new ChartJS(barCtx, {
        type: 'bar',
        data: getBarData(),
        options: barOptions
      })
    }
  })

  // Update charts when data changes
  afterUpdate(() => {
    if (pieChart) {
      pieChart.data = getPieData()
      pieChart.update()
    }

    if (barChart) {
      barChart.data = getBarData()
      barChart.update()
    }
  })

  // Handle cleanup
  function onDestroy() {
    if (pieChart) {
      pieChart.destroy()
      pieChart = null
    }
    if (barChart) {
      barChart.destroy()
      barChart = null
    }
  }
</script>

<div class="spending-chart">
  <h2>Spending Analysis</h2>
  
  <div class="chart-container">
    <div class="chart-wrapper">
      <h3>By Category</h3>
      <div class="chart-inner">
        <canvas bind:this={pieCtx}></canvas>
      </div>
    </div>
    
    <div class="chart-wrapper">
      <h3>Monthly Breakdown</h3>
      <div class="chart-inner">
        <canvas bind:this={barCtx}></canvas>
      </div>
    </div>
  </div>
</div>

<style>
  .spending-chart {
    background: white;
    border-radius: 12px;
    padding: 2rem;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .spending-chart h2 {
    margin: 0 0 2rem 0;
    font-size: 1.5rem;
    font-weight: 600;
    color: #333;
  }

  .chart-container {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 2rem;
  }

  .chart-wrapper {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .chart-wrapper h3 {
    margin: 0;
    font-size: 1.1rem;
    font-weight: 600;
    color: #333;
    text-align: center;
  }

  .chart-inner {
    height: 300px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  @media (max-width: 768px) {
    .spending-chart {
      padding: 1.5rem;
    }

    .chart-container {
      grid-template-columns: 1fr;
    }

    .chart-inner {
      height: 250px;
    }
  }
</style>
