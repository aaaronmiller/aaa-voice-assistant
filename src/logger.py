import logging

# Configure logging
logging.basicConfig(
    filename='assistant.log',
    filemode='a',
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    level=logging.INFO
)

def log_error(e):
    logging.error(f"Exception occurred: {e}", exc_info=True)
