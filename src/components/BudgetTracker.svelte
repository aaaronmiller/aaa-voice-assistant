<script>
  export let budgets = []

  // Calculate progress percentage for each budget
  const getProgress = (budget, spent) => {
    return Math.min((spent / budget) * 100, 100)
  }

  // Get progress bar color based on spending percentage
  const getProgressColor = (percentage) => {
    if (percentage < 70) return '#10b981' // green
    if (percentage < 90) return '#f59e0b' // orange
    return '#ef4444' // red
  }
</script>

<div class="budget-tracker">
  <h2>Budget Tracking</h2>
  
  <div class="budgets-list">
    {#each props.budgets as budget}
      <div class="budget-item">
        <div class="budget-header">
          <div class="budget-info">
            <h3>{budget.category}</h3>
            <p class="budget-details">
              ${budget.spent.toFixed(2)} of ${budget.budget.toFixed(2)}
            </p>
          </div>
          <div class="budget-percentage">
            {Math.round((budget.spent / budget.budget) * 100)}%
          </div>
        </div>
        
        <div class="progress-bar">
          <div 
            class="progress-fill"
            style={`
              width: ${getProgress(budget.budget, budget.spent)}%;
              background-color: ${getProgressColor(getProgress(budget.budget, budget.spent))};
            `}
          ></div>
        </div>
        
        <div class="budget-remaining">
          Remaining: ${(budget.budget - budget.spent).toFixed(2)}
        </div>
      </div>
    {/each}
  </div>

  <div class="budget-summary">
    <div class="summary-item">
      <span class="label">Total Budget:</span>
      <span class="value">
        ${props.budgets.reduce((sum, b) => sum + b.budget, 0).toFixed(2)}
      </span>
    </div>
    <div class="summary-item">
      <span class="label">Total Spent:</span>
      <span class="value">
        ${props.budgets.reduce((sum, b) => sum + b.spent, 0).toFixed(2)}
      </span>
    </div>
    <div class="summary-item">
      <span class="label">Total Remaining:</span>
      <span class="value">
        ${props.budgets.reduce((sum, b) => sum + (b.budget - b.spent), 0).toFixed(2)}
      </span>
    </div>
  </div>
</div>

<style>
  .budget-tracker {
    background: white;
    border-radius: 12px;
    padding: 2rem;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .budget-tracker h2 {
    margin: 0 0 2rem 0;
    font-size: 1.5rem;
    font-weight: 600;
    color: #333;
  }

  .budgets-list {
    display: flex;
    flex-direction: column;
    gap: 2rem;
    margin-bottom: 2rem;
  }

  .budget-item {
    padding: 1.5rem;
    background: #f8f9fa;
    border-radius: 8px;
  }

  .budget-header {
    display: flex;
    justify-content: space-between;
    align-items: start;
    margin-bottom: 1rem;
  }

  .budget-info h3 {
    margin: 0 0 0.5rem 0;
    font-size: 1.1rem;
    font-weight: 600;
    color: #333;
  }

  .budget-details {
    margin: 0;
    font-size: 0.9rem;
    color: #666;
  }

  .budget-percentage {
    font-size: 1.2rem;
    font-weight: 600;
    color: #667eea;
  }

  .progress-bar {
    width: 100%;
    height: 8px;
    background: #e0e0e0;
    border-radius: 4px;
    overflow: hidden;
    margin-bottom: 1rem;
  }

  .progress-fill {
    height: 100%;
    border-radius: 4px;
    transition: width 0.3s ease;
  }

  .budget-remaining {
    font-size: 0.9rem;
    color: #666;
  }

  .budget-summary {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
    padding-top: 1.5rem;
    border-top: 1px solid #e0e0e0;
  }

  .summary-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .summary-item .label {
    font-size: 0.9rem;
    color: #666;
    font-weight: 500;
  }

  .summary-item .value {
    font-size: 1.1rem;
    font-weight: 600;
    color: #333;
  }

  @media (max-width: 768px) {
    .budget-tracker {
      padding: 1.5rem;
    }

    .budget-summary {
      grid-template-columns: 1fr;
    }
  }
</style>
