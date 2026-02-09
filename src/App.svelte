<script>
  import BudgetTracker from './components/BudgetTracker.svelte'
  import ExpenseManager from './components/ExpenseManager.svelte'
  import SpendingChart from './components/SpendingChart.svelte'

  // Mock data
  const categories = ['Food', 'Transport', 'Shopping', 'Entertainment', 'Bills']
  const budget = [
    { category: 'Food', budget: 500, spent: 420 },
    { category: 'Transport', budget: 200, spent: 180 },
    { category: 'Shopping', budget: 300, spent: 150 },
    { category: 'Entertainment', budget: 150, spent: 120 },
    { category: 'Bills', budget: 800, spent: 800 }
  ]

  const expenses = [
    { id: 1, description: 'Groceries', amount: 85.50, category: 'Food', date: '2024-10-01' },
    { id: 2, description: 'Uber', amount: 15.00, category: 'Transport', date: '2024-10-02' },
    { id: 3, description: 'Netflix', amount: 15.99, category: 'Entertainment', date: '2024-10-03' },
    { id: 4, description: 'Electricity', amount: 120.00, category: 'Bills', date: '2024-10-04' },
    { id: 5, description: 'Coffee', amount: 4.50, category: 'Food', date: '2024-10-05' },
    { id: 6, description: 'Shopping', amount: 45.00, category: 'Shopping', date: '2024-10-06' },
    { id: 7, description: 'Gas', amount: 35.00, category: 'Transport', date: '2024-10-07' },
    { id: 8, description: 'Dinner', amount: 65.00, category: 'Food', date: '2024-10-08' }
  ]

  // Current view state
  let currentView = $state('dashboard')
</script>

<main class="app-container">
  <!-- Header -->
  <header class="app-header">
    <div class="header-content">
      <h1>Financial Dashboard</h1>
      <nav class="nav-menu">
        <button 
          class="nav-btn {currentView === 'dashboard' ? 'active' : ''}"
          onclick={() => currentView = 'dashboard'}
        >
          Dashboard
        </button>
        <button 
          class="nav-btn {currentView === 'budgets' ? 'active' : ''}"
          onclick={() => currentView = 'budgets'}
        >
          Budgets
        </button>
        <button 
          class="nav-btn {currentView === 'expenses' ? 'active' : ''}"
          onclick={() => currentView = 'expenses'}
        >
          Expenses
        </button>
      </nav>
    </div>
  </header>

  <!-- Main Content -->
  <section class="main-content">
    {#if currentView === 'dashboard'}
      <div class="dashboard">
        <div class="overview-cards">
          <div class="card">
            <h3>Total Income</h3>
            <p class="amount positive">$4,500</p>
          </div>
          <div class="card">
            <h3>Total Expenses</h3>
            <p class="amount negative">$1,790</p>
          </div>
          <div class="card">
            <h3>Savings</h3>
            <p class="amount positive">$2,710</p>
          </div>
        </div>
        <SpendingChart {expenses} />
        <BudgetTracker {budgets} />
      </div>
    {:else if currentView === 'budgets'}
      <BudgetTracker {budgets} />
    {:else if currentView === 'expenses'}
      <ExpenseManager {expenses} {categories} />
    {/if}
  </section>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
    background-color: #f5f7fa;
    color: #333;
  }

  .app-container {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
  }

  .app-header {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    padding: 1rem 0;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  }

  .header-content {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 2rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .app-header h1 {
    margin: 0;
    font-size: 1.8rem;
    font-weight: 300;
  }

  .nav-menu {
    display: flex;
    gap: 1rem;
  }

  .nav-btn {
    background: rgba(255, 255, 255, 0.2);
    border: none;
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.9rem;
    transition: background 0.3s;
  }

  .nav-btn:hover,
  .nav-btn.active {
    background: rgba(255, 255, 255, 0.3);
  }

  .main-content {
    flex: 1;
    max-width: 1200px;
    width: 100%;
    margin: 0 auto;
    padding: 2rem;
    box-sizing: border-box;
  }

  .dashboard {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 2rem;
  }

  .overview-cards {
    grid-column: 1 / -1;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1.5rem;
  }

  .card {
    background: white;
    border-radius: 12px;
    padding: 1.5rem;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .card h3 {
    margin: 0 0 1rem 0;
    font-size: 0.9rem;
    font-weight: 500;
    color: #666;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .amount {
    font-size: 2rem;
    font-weight: 300;
    margin: 0;
  }

  .amount.positive {
    color: #10b981;
  }

  .amount.negative {
    color: #ef4444;
  }

  @media (max-width: 768px) {
    .header-content {
      flex-direction: column;
      gap: 1rem;
    }

    .nav-menu {
      flex-wrap: wrap;
      justify-content: center;
    }

    .main-content {
      padding: 1rem;
    }

    .dashboard {
      grid-template-columns: 1fr;
    }

    .overview-cards {
      grid-template-columns: 1fr;
    }
  }
</style>
