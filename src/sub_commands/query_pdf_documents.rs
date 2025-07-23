use anyhow::Result;

use crate::RagSystem;

pub async fn query(rag: &RagSystem) -> Result<()> {
    // Example queries: https://chatgpt.com/c/688107d4-8248-8322-984a-d3afe03fd5a2
    let queries = vec![
        "What is the central thesis of \"Moore’s Law for Everything\"?",
        // Answer: The central thesis is that AI will dramatically reduce the cost of labor, leading to an abundance of wealth, and society must adapt by fairly distributing this wealth—particularly through taxing capital assets like companies and land—to ensure broad participation in its benefits.
        "According to the author, what will AI systems be capable of in the next 5 to 10 years?",
        // Answer: In the next five years, AI will be able to read legal documents and give medical advice. In ten years, it may perform assembly-line work and serve as companions.
        "Why does the author argue that taxing labor will become less effective in the future?",
        // Answer: Because many future jobs won't generate significant economic value as AI will handle most basic goods and services production, making labor a less relevant source of wealth.
        "What are the two main types of assets proposed for taxation in the American Equity Fund?",
        // Answer: The two main assets are high-valuation companies (taxed via shares) and privately-held land (taxed in cash).
        "How does the author suggest handling the risk of companies avoiding the equity tax by offshoring?",
        // Answer: By applying a revenue-based test to determine if a company derives a significant portion of its business from the U.S., thereby still subjecting it to the tax.
        "What is meant by 'Moore’s Law for everything'?",
        // Answer: It refers to the idea that the cost of goods and services—not just computing—can decrease exponentially, making essential needs more affordable over time due to technological advances like AI.
        "How does the author propose preventing citizens from misusing future distributions from the Fund?",
        // Answer: By making it legally unenforceable to sell, borrow against, or pledge their future distributions.
        "What historical example does the author reference to show that large social reforms can be rapidly adopted?",
        // Answer: The implementation of a massive social safety net during the Great Depression under Franklin Roosevelt.
        "What is the purpose of tying the full implementation of the 2.5% tax to a 50% increase in GDP?",
        // Answer: To reduce transitional shock and make the reform politically and economically feasible by aligning it with clear national growth milestones.
        "How does the author contrast “more good” versus “less bad” approaches to societal progress?"
        // Answer: “More good” focuses on expanding opportunities and growing societal wealth, while “less bad” emphasizes reducing inequality through redistribution—both are important, but sustained progress comes from focusing on growth.
    ];

    // Query the RAG system
    for query in queries {
        println!("\n{}", "=".repeat(60));
        println!("Question: {}", query);

        match rag.query(query).await {
            Ok(answer) => println!("Answer: {}", answer),
            Err(e) => println!("Error: {}", e),
        }
    }
    Ok(())
}
