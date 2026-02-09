<script>
  export let expenses = []
  export let categories = []

  // Form state
  let newExpense = {
    description: '',
    amount: '',
    category: categories[0],
    date: new Date().toISOString().split('T')[0]
  }

  // Filter state
  let filter = {
    category: 'all',
    sortBy: 'date'
  }

  // Add new expense
  function addExpense() {
    if (!newExpense.description || !newExpense.amount) return

    const expense = {
      id: Date.now(),
      description: newExpense.description,
      amount: parseFloat(newExpense.amount),
      category: newExpense.category,
      date: newExpense.date
    }

    props.expenses.push(expense)
    
    // Reset form
    newExpense.description = ''
    newExpense.amount = ''
    newExpense.date = new Date().toISOString().split('T')[0]
  }

  // Delete expense
  function deleteExpense(id) {
    const index = props.expenses.findIndex(exp => exp.id === id)
    if (index > -1) {
      props.expenses.splice(index, 1)
    }
  }

  // Filtered expenses
  const filteredExpenses = $derived(() => {
    let result = [...props.expenses]

    // Filter by category
    if (filter.category !== 'all') {
      result = result.filter(exp => exp.category === filter.category)
    }

    // Sort expenses
    result.sort((a, b) => {
      if (filter.sortBy === 'date') {
        return new Date(b.date) - new Date(a.date)
      } else if (filter.sortBy === 'amount') {
        return b.amount - a.amount
      } else if (filter.sortBy === 'category') {
        return a.category.localeCompare(b.category)
      }
      return 0
    })

    return result
  })

  // Total expenses
  const totalExpenses = $derived(() => {
    return filteredExpenses.reduce((sum, exp) => sum + exp.amount, 0)
  })

  // Get category color
  function getCategoryColor(category) {
    const colors = {
      Food: '#ef4444',
      Transport: '#f59e0b',
      Shopping: '#10b981',
      Entertainment: '#3b82f6',
      Bills: '#6366f1'
    }
    return colors[category] || '#6b7280'
  }
</script>

<div class="expense-manager">
  <h2>Expense Management</h2>

  <!-- Add Expense Form -->
  <div class="add-expense-form">
    <h3>Add New Expense</h3>
    <div class="form-grid">
      <div class="form-group">
        <label>Description</label>
        <input
          type="text"
          bind:value={newExpense.description}
          placeholder="Enter description"
        />
      </div>
      <div class="form-group">
        <label>Amount ($)</label>
        <input
          type="number"
          bind:value={newExpense.amount}
          placeholder="0.00"
          step="0.01"
          min="0"
        />
      </div>
      <div class="form-group">
        <label>Category</label>
        <select bind:value={newExpense.category}>
          {#each props.categories as category}
            <option value={category}>{category}</option>
          {/each}
        </select>
      </div>
      <div class="form-group">
        <label>Date</label>
        <input
          type="date"
          bind:value={newExpense.date}
        />
      </div>
    </div>
    <button class="add-btn" onclick={addExpense}>
      Add Expense
    </button>
  </div>

  <!-- Filters -->
  <div class="filters">
    <div class="filter-group">
      <label>Category:</label>
      <select bind:value={filter.category}>
        <option value="all">All Categories</option>
        {#each props.categories as category}
          <option value={category}>{category}</option>
        {/each}
      </select>
    </div>
    <div class="filter-group">
      <label>Sort By:</label>
      <select bind:value={filter.sortBy}>
        <option value="date">Date (Newest)</option>
        <option value="amount">Amount (Highest)</option>
        <option value="category">Category</option>
      </select>
    </div>
  </div>

  <!-- Expenses List -->
  <div class="expenses-list">
    <div class="list-header">
      <h3>Recent Expenses</h3>
      <div class="total">
        Total: <span class="total-amount">${totalExpenses.toFixed(2)}</span>
      </div>
    </div>
    
    {#if filteredExpenses.length === 0}
      <div class="empty-state">
        No expenses found
      </div>
    {:else}
      {#each filteredExpenses as expense}
        <div class="expense-item">
          <div class="expense-info">
            <div class="expense-description">{expense.description}</div>
            <div class="expense-meta">
              <span 
                class="category-badge"
                style="background-color: {getCategoryColor(expense.category)}"
              >
                {expense.category}
              </span>
              <span class="expense-date">{expense.date}</span>
            </div>
          </div>
          <div class="expense-actions">
            <div class="expense-amount">${expense.amount.toFixed(2)}</div>
            <button 
              class="delete-btn"
              onclick={() => deleteExpense(expense.id)}
              title="Delete expense"
            >
              ×
            </button>
          </div>
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .expense-manager {
    background: white;
    border-radius: 12px;
    padding: 2rem;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .expense-manager h2 {
    margin: 0 0 2rem 0;
    font-size: 1.5rem;
    font-weight: 600;
    color: #333;
  }

  .add-expense-form {
    margin-bottom: 2rem;
    padding: 1.5rem;
    background: #f8f9fa;
    border-radius: 8px;
  }

  .add-expense-form h3 {
    margin: 0 0 1.5rem 0;
    font-size: 1.1rem;
    font-weight: 600;
    color: #333;
  }

  .form-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
    margin-bottom: 1.5rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
  }

  .form-group label {
    margin-bottom: 0.5rem;
    font-size: 0.9rem;
    font-weight: 500;
    color: #666;
  }

  .form-group input,
  .form-group select {
    padding: 0.75rem;
    border: 1px solid #e0e0e0;
    border-radius: 6px;
    font-size: 0.9rem;
    background: white;
  }

  .add-btn {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.9rem;
    font-weight: 500;
    transition: opacity 0.3s;
  }

  .add-btn:hover {
    opacity: 0.9;
  }

  .filters {
    display: flex;
    gap: 1rem;
    margin-bottom: 1.5rem;
    flex-wrap: wrap;
  }

  .filter-group {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .filter-group label {
    font-size: 0.9rem;
    font-weight: 500;
    color: #666;
  }

  .filter-group select {
    padding: 0.5rem;
    border: 1px solid #e0e0e0;
    border-radius: 6px;
    font-size: 0.9rem;
    background: white;
  }

  .expenses-list {
    margin-top: 2rem;
  }

  .list-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .list-header h3 {
    margin: 0;
    font-size: 1.1rem;
    font-weight: 600;
    color: #333;
  }

  .total {
    font-size: 0.9rem;
    color: #666;
  }

  .total-amount {
    font-size: 1.1rem;
    font-weight: 600;
    color: #ef4444;
  }

  .empty-state {
    text-align: center;
    padding: 3rem;
    color: #666;
    background: #f8f9fa;
    border-radius: 8px;
  }

  .expense-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.25rem;
    border-bottom: 1px solid #f0f0f0;
  }

  .expense-item:last-child {
    border-bottom: none;
  }

  .expense-info {
    flex: 1;
  }

  .expense-description {
    font-weight: 500;
    color: #333;
    margin-bottom: 0.5rem;
  }

  .expense-meta {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }

  .category-badge {
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.8rem;
    font-weight: 500;
    color: white;
  }

  .expense-date {
    font-size: 0.8rem;
    color: #999;
  }

  .expense-actions {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .expense-amount {
    font-weight: 600;
    color: #333;
    font-size: 1.1rem;
  }

  .delete-btn {
    background: #ef4444;
    color: white;
    border: none;
    width: 28px;
    height: 28px;
    border-radius: 50%;
    cursor: pointer;
    font-size: 1.2rem;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 0.3s;
  }

  .delete-btn:hover {
    background: #dc2626;
  }

  @media (max-width: 768px) {
    .expense-manager {
      padding: 1.5rem;
    }

    .form-grid {
      grid-template-columns: 1fr;
    }

    .filters {
      flex-direction: column;
    }

    .expense-item {
      flex-direction: column;
      align-items: start;
      gap: 1rem;
    }
  }
</style>
